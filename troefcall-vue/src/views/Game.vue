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
    <v-container class="d-flex justify-center align-center" v-else>
        <div v-if="gameInfo.length > 0">
            <p class="text-subtitle-2"> {{ gameInfo }} </p>
            <Table v-if="displayTable" :game="game?.value" />
            <Cards v-if="displayCards" :cards="cards" />
        </div>
    </v-container>
    <!-- forfeit button -->
    <div class="d-flex justify-center align-center" v-if="canForfeit">
        <v-btn class="ma-16" color="error" @click="forfeit">Forfeit</v-btn>
    </div>
</template>

<script setup>
import { onMounted } from 'vue';
import { ref } from 'vue';
import { get, post, del } from '@/requests';
import { useAuthStore } from '@/store/auth';
import Table from '@/components/troefcall/game/Table.vue';
import Cards from '@/components/troefcall/game/Cards.vue';
import { onBeforeUnmount } from 'vue';

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
    console.log('Mounted game');

    refresh();
    const REFRESH_INTERVAL = 5000; //ms
    refresher = setInterval(refresh, REFRESH_INTERVAL);

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
            PlayerIsStarting();
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
        gameInfo.value = `Game is finished: ${game?.value?.winner?.name} won!`;
    }
    else {
        gameInfo.value = 'Game is in unknown state';
    }
}

function PlayerIsStarting() {
    gameInfo.value = 'You are starting: pick a tjall (trump) suit';
    get(`rooms/${props.roomId}/game/cards`)
        .then(response => {
            cards.value = response.data;
            console.log('Cards: ' + JSON.stringify(cards.value));
            gameInfo.value = 'You are starting: pick a tjall (trump) suit';
        }).catch(error => {
            let errorMessage = error.response?.data ?? 'No response data';
            gameDataError.value = errorMessage;
            console.error('Error while refreshing game data:\n' + gameDataError.value);
            updateState();
        });
    displayCards.value = true;
    displayTable.value = false;
    //get cards
}

function forfeit(){
    del(`rooms/${props.roomId}/game`)
        .then(response => {
            console.log('Forfeited game: ' + JSON.stringify(response.data));
            router.push('/rooms/' + props.roomId);
        }).catch(error => {
            console.error('Failed to forfeit game: ' + JSON.stringify(error.response));
        });
}

function isStateStarting() {
    return game?.value?.state?.hasOwnProperty("Starting");
}
function isStatePlaying() {
    return game?.value?.state?.hasOwnProperty("Playing");
}
function isStateFinished() {
    return game?.value?.state?.hasOwnProperty("Finished");
}

function isPlayer() {
    return auth.isInAnyRoom && auth.getRoomId == props.roomId;
}

function canForfeit() {
    return isPlayer() && game.value != null && !isStateFinished();
}

function isYourTurn() {
    if (this.game == null) {
        return false;
    }
    let playerIndex = 0;
    if (this.game.played_rounds.length > 0) {
        //get last in played_rounds array
        let currentRound = this.game.played_rounds[this.game.played_rounds.length - 1];
        if (currentRound.length < 4) {
            playerIndex = currentRound.length;
        }
    }
    return this.game.players[playerIndex] == auth.user.id;
}
</script>
