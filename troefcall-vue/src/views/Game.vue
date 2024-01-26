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
    <div v-else-if="game?.value === null">
        <v-alert type="info">
            Game has not started yet.
        </v-alert>
    </div>
    <v-card class="d-flex flex-column ma-1 pa-1" :disabled="!isYourTurn" v-else>
        <div v-if="gameInfo.length > 0">
            <p class="text-subtitle-2"> {{ gameInfo }} </p>
        </div>
        <v-btn v-if="isStateFinished()" class="ma-3" color="success" text="Back to room"
            :to="{ name: 'Room', params: { id: props.roomId } }" />
        <div>
            <RoundTable v-if="displayTable" :game="game?.value" />
            <Cards v-if="displayCards" :cards="cards" />
        </div>
    </v-card>
    <!-- forfeit button -->
    <div class="d-flex justify-center align-center" v-if="canForfeit()">
        <v-btn class="ma-16" color="error" @click="forfeit">Forfeit</v-btn>
    </div>
</template>

<script setup>
/* eslint-disable no-prototype-builtins */
import { onMounted } from 'vue';
import { ref } from 'vue';
import { get, post, del } from '@/requests';
import { useAuthStore } from '@/store/auth';
import RoundTable from '@/components/troefcall/game/RoundTable.vue';
import Cards from '@/components/troefcall/game/Cards.vue';
import { onBeforeUnmount } from 'vue';
import { GAME_REFRESH_INTERVAL } from '@/store/preferences';

const props = defineProps({
    roomId: {
        type: Number,
        required: true,
    },
});

const auth = useAuthStore();

const game = ref(null);
const cards = ref([]);

const displayTable = ref(false);
const displayCards = ref(false);
const gameInfo = ref("");
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

async function refresh() {
    console.log('Refreshing game data');
    await get(`rooms/${props.roomId}/game`)
        .then(response => {
            game.value = response.data;
            gameDataError.value = false;
            console.log('Game data refreshed successfully!\n' + JSON.stringify(game.value, null, 2));
            updateState();
        }).catch(error => {
            let errorMessage = error.response?.data ?? 'No response data';
            gameDataError.value = errorMessage;
            console.error('Error while refreshing game data:\n' + gameDataError.value);
            updateState();
        });
}

function updateState() {
    if (isStateStarting()) {
        const startingPlayer = game?.value?.players[0];
        console.log('Starting player: ' + JSON.stringify(startingPlayer, null, 2));
        //check if player is starting
        if (startingPlayer?.user_id === auth.user?.id) {
            console.log('You are starting');
            onPlayerIsStarting();
        }
        else {
            gameInfo.value = `Player ${game?.value?.players[0].name} is starting`;
        }
    }
    else if (isStatePlaying()) {
        //check if player is starting
        if (isYourTurn()) {
            gameInfo.value = 'It is your turn';
        }
        else {
            // const player = game?.value?.players[0]?.name;
            let player = JSON.stringify(game?.value?.players[0], null, 2);
            gameInfo.value = `Player ${player} is playing`;
        }
    }
    else if (isStateFinished()) {
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

function onPlayerIsStarting() {
    gameInfo.value = 'You are starting: pick a tjall (trump) suit';
    get(`rooms/${props.roomId}/game/cards`)
        .then(response => {
            setCards(response);
            gameInfo.value = 'You are starting: pick a tjall (trump) suit';
        }).catch(error => {
            let errorMessage = error.response?.data ?? 'No response data';
            gameDataError.value = errorMessage;
            console.error('Error while getting cards:\n' + gameDataError.value);
        });
    displayCards.value = true;
    displayTable.value = false;
    //get cards
}

function setCards(response) {
    cards.value = response.data;
    cards.value.sort((a, b) => {
        if (a.suit === b.suit) {
            return a.value - b.value;
        }
        else {
            if (a.suit.charAt(0) === 'H' || a.suit.charAt(0) === 'D') {
                return -1;
            }
            else if (b.suit.charAt(0) === 'H' || b.suit.charAt(0) === 'D') {
                return 1;
            }
        }
    });
    console.log('Cards: ' + JSON.stringify(cards.value));
}

function forfeit() {
    del(`rooms/${props.roomId}/game`)
        .then(response => {
            console.log('Forfeited game: ' + JSON.stringify(response.data));
        }).catch(error => {
            console.error('Failed to forfeit game: ' + JSON.stringify(error.response?.data ?? error.response ?? error));
        });
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
function isPlayer() {
    return auth.isInAnyRoom && auth.getRoomId == props.roomId;
}
function isYourTurn() {
    if (this.game == null) {
        return false;
    }
    let playerIndex = 0;
    const played_rounds = this.game.played_rounds;
    if (played_rounds.length > 0) {
        //get last in played_rounds array
        let currentRound = played_rounds[played_rounds.length - 1];
        if (currentRound.length < 4) {
            playerIndex = currentRound.length;
        }
    }
    return this.game.players[playerIndex] == auth.user.id;
}

</script>