<!-- eslint-disable vue/multi-word-component-names -->
<template>
    <div v-if="!isPlayer">
        <v-alert type="info">
            You are not in this room. <router-link to='/rooms'>Go back to rooms</router-link>
        </v-alert>
    </div>
    <div v-else-if="gameDataError === null">
        <v-alert type="info">
            Loading game data...
        </v-alert>
    </div>
    <div v-else-if="gameDataError !== false">
        <v-alert type="error">
            Failed to get game data: {{ gameDataError }}
        </v-alert>
    </div>
    <div v-else-if="game.value === null">
        <v-alert type="info">
            Game has not started yet.
        </v-alert>
    </div>

    <div class="d-flex flex-column ma-1 pa-1 align-center" v-else>
        <div class="d-flex flex-wrap ma-1 pa-1 align-center">
            <ScoreDisplay v-if="!isStateStarting()" :game="game"/>
            <v-card v-if="previousRound" >
                <v-card-title>
                    <p class="text-h6">Last round:</p>
                </v-card-title>
                <RoundTable :compact="true" :round="previousRound" :cards="cards" :players="game.players" />
            </v-card>
            <RoundTable v-if="currentRound" :round="currentRound" :cards="cards" :players="game.players" />
        </div>

        <div v-if="gameInfo != null" class="d-flex justify-center ">
            <p class="text-subtitle-2"> {{ gameInfo }} </p>
            <br/>
        </div>
        <v-btn v-if="isStateFinished()" class="ma-3" color="success" text="Back to room" :to="{ name: 'Room', params: { id: props.roomId } }" />
        <Cards v-if="canSeeCards()" :disabled="!isYourTurn()" :cards="cards" :tjall="getTjall()" @onPlayCard="playCard" @onSelect="errorMessage = null" />
        <v-alert v-if="errorMessage" type="error">
            {{ errorMessage }}
        </v-alert>
    </div>

    <div class="d-flex justify-center align-center" v-if="canForfeit()">
        <v-btn v-if="isPlayer()" class="ma-16" color="error" @click="forfeit">Forfeit</v-btn>
    </div>
</template>

<script setup>
/* eslint-disable no-prototype-builtins */
import { onMounted, onBeforeUnmount, ref, watch } from 'vue';

import { get, put, del } from '@/requests';
import { useAuthStore } from '@/store/auth';
import { GAME_REFRESH_INTERVAL } from '@/store/preferences';

import RoundTable from '@/components/troefcall/game/RoundTable.vue';
import Cards from '@/components/troefcall/game/Cards.vue';
import ScoreDisplay from '@/components/troefcall/game/ScoreDisplay.vue';

const props = defineProps({
    roomId: {
        type: Number,
        required: true,
    },
});

const auth = useAuthStore();

const game = ref(null);
const cards = ref([]);

const currentRound = ref(false);
const previousRound = ref(false);
const gameInfo = ref(null);
const errorMessage = ref(null);
const gameDataError = ref(null);

let refresher = null;

onMounted(() => {
    refresh();
    refresher = setInterval(refresh, GAME_REFRESH_INTERVAL);
    gameDataError.value = null;
});
onBeforeUnmount(() => {
    console.log('Unmounted game');
    clearInterval(refresher);
});

watch(() => auth.user, 
            () => { refresh(); }
);


async function refresh() {
    console.log('Refreshing game data');
    await get(`rooms/${props.roomId}/game`)
    .then(response => {
            game.value = response.data;
            console.log('Game data refreshed successfully!\n' + JSON.stringify(game.value, null, 2));
            gameDataError.value = false;
            getCards();
            updateState();
        }).catch(error => {
            let errorMessage = error.response?.data ?? 'No response data';
            gameDataError.value = errorMessage;
            console.error('Error while refreshing game data:\n' + gameDataError.value);
            updateState();
        });
}

function playCard(card) {
    console.log('Playing card: ' + JSON.stringify(card, null, 2));
    put(`/rooms/${props.roomId}/game`, card)
        .then(response => {
            console.log('Card played successfully: ' + JSON.stringify(response.data));
            game.value = response.data;
            if (isStatePlaying()) {
                cards.value = cards.value
                    .filter(c => c.suit != card.suit || c.value != card.value);
                }
            else if (isStateStarting()) {
                getCards();
            }
            updateState();
        }).catch(error => {
            let errMsg = JSON.stringify(error.response?.data ?? error.response ?? error);
            if(errMsg.startsWith('"')){
                errMsg = errMsg.substring(1, errMsg.length - 1);
            }
            errorMessage.value = errMsg;
            console.error(errorMessage.value);
        });
}

function forfeit() {
    del(`rooms/${props.roomId}/game`)
        .then(response => {
            console.log('Forfeited game: ' + JSON.stringify(response.data));
            game.value = response.data;
            updateState();
        }).catch(error => {
            console.error('Failed to forfeit game: ' + JSON.stringify(error.response?.data ?? error.response ?? error));
        });
}
function updateState() {
    if (isStateStarting()) {
        const startingPlayer = game?.value?.players[0];
        if (startingPlayer == null || startingPlayer == undefined) {
            console.error('Starting player is null');
            return;
        }
        console.log('Game is starting. Starting player: ' + JSON.stringify(startingPlayer, null, 2));
        //check if player is starting
        if (startingPlayer.user_id == auth.user.id) {
            console.log('You are starting');
            gameInfo.value = 'You are starting: pick a tjall (trump) suit';
        }
        else {
            console.log('Player ' + startingPlayer.name + ' is starting');
            gameInfo.value = `Player ${game?.value?.players[0].name} is starting.. Please wait for them to pick a tjall.`;
        }
        currentRound.value = null;
    }
    else if (isStatePlaying()) {
        currentRound.value = game.value.state.Playing.current_round;
        const played_rounds = game.value.played_rounds;
        if (played_rounds == null || played_rounds == undefined){
            console.error('Played rounds is null or undefined');
            return;
        }
        if (played_rounds.length > 0) {
            previousRound.value = played_rounds[played_rounds.length - 1];
        }
        else {
            previousRound.value = null;
        }
        //check if player is starting
        console.log('Game state is playing');
        if (isYourTurn()) {
            gameInfo.value = 'It is your turn';
        }
        else {
            // const player = game?.value?.players[0]?.name;
            gameInfo.value = `Player ${getCurrentTurnPlayer().name} is playing`;
        }
    }
    else if (isStateFinished()) {
        currentRound.value = null;
        console.log('Game is finished');
        const newLocal = game?.value?.state;
        //check reason
        let state = newLocal?.Finished;
        let winners = state?.winners.map(winner => winner.name).join(' and ');
        let gameFinishedMessage = `Game is finished: ${winners} won!`;
        if (state?.reason.hasOwnProperty('Forfeit')) {
            gameFinishedMessage = `Game is finished: ${winners} won! (Forfeit by ${state.reason.Forfeit.player.name})`;
        }
        gameInfo.value = gameFinishedMessage;

    }
    else {
        gameInfo.value = 'Game is in unknown state';
    }
}

function getCards() {
    if(game.value == null){
        console.error('Game is null, cannot get cards');
        return;
    }
    if(!auth.loggedIn){
        console.warn('User is not logged in, cannot get cards');
        return;
    }
    return get(`rooms/${props.roomId}/game/cards`)
        .then(response => {
            cards.value = response.data.sort((a, b) => {
                if (a.suit === b.suit) {
                    return a.value - b.value;
                }
                else {
                    const suits = ['S', 'H', 'C', 'D'];
                    let tjall = suits.indexOf(getTjall()?.charAt(0));
                    if (tjall == -1) {
                        tjall = 0;
                    }
                    //rotate suits array so that tjall is first
                    suits.unshift(...suits.splice(tjall));
                    return suits.indexOf(a.suit.charAt(0)) - suits.indexOf(b.suit.charAt(0));
                }
            });
            console.log('Cards: ' + JSON.stringify(cards.value));
        }).catch(error => {
            let errorMessage = error.response?.data ?? 'No response data';
            console.error('Error while getting cards:\n' + errorMessage);
        });
}
function getTjall() {
    return game.value.state?.Playing?.tjall;
}

function isStateStarting() {
    return game?.value?.state?.hasOwnProperty("Starting")
}
function isStatePlaying() {
    return game?.value?.state?.hasOwnProperty("Playing");
}
function isStateFinished() {
    return game?.value?.state?.hasOwnProperty("Finished")
}

function canForfeit() {
    return isPlayer() && game.value != null && !isStateFinished();
}
function canSeeCards() {
    if (!isPlayer()) { return false; }
    if (isStateStarting()) { return getCurrentTurnPlayer()?.user_id == auth.user?.id; }
    if (isStatePlaying()) { return true; }
    return false;
}
function isPlayer() {
    if (!auth.loggedIn){
        return false;
    }
    return auth.isInAnyRoom.then((isInRoom) => {
        if (isInRoom) {
            const isInThisRoom = auth.getRoomId == props.roomId;
            return isInThisRoom;
        }
        return false;
    })
    .catch((error) => {
        console.error('Failed to check if user is player: ' + error);
        return false;
    });
}
function isYourTurn() {
    if (game.value == null) {
        console.error('Game is null, cannot check if it is your turn');
        return false;
    }
    return getCurrentTurnPlayer()?.user_id == auth.user?.id;

}
function getCurrentTurnPlayer() {
    let playerIndex = 0;
    if (currentRound.value != null) {
        const cards = currentRound.value.played_cards;
        playerIndex = (cards.length + currentRound.value.player_starting) % 4;	
    }
    console.log('Current round: ' + JSON.stringify(currentRound.value, null, 2) + '\nPlayer index: ' + playerIndex);
    const current_turn_player = game.value.players[playerIndex];
    return current_turn_player;
}

</script>