<template>
    <div v-if="round">
        <v-card v-if="!compact" width="350"
            color="#005500"
            class="ma-1 pa-1"
            rounded="lg"
        >   
            <v-container class="d-flex align-content-center justify-center">
                <PlayerCard :name="getPlayerName(0)" :card="getCard(0)" />
            </v-container>
            <v-container class="d-flex align-content-center justify-space-between">
                <PlayerCard :name="getPlayerName(3)" :card="getCard(3)" />
                <PlayerCard :name="getPlayerName(1)" :card="getCard(1)" />
            </v-container>
            <v-container class="d-flex align-content-center justify-center">
                <PlayerCard :name="getPlayerName(2)" :card="getCard(2)" />
            </v-container>
        </v-card>
        <v-card v-else 
                width="150" class="ma-0 pa-0" density="compact"
                color="#005500" rounded="lg">
            <v-card-text>
                <!-- compact cards display -->
                <div>
                    <v-card v-for="(card, index) in round.played_cards" :key="index" >
                        <div class="d-flex justify-center align-center">
                            <v-spacer/>
                            <p class="text-truncate">
                                {{ getPlayerName(index) }}
                            </p>
                            <Card :card="card" :simple="true"/>
                        </div>
                    </v-card>
                </div>
            </v-card-text>
        </v-card>
    </div>
    <div v-else>
        <v-alert type="error">Round is null!</v-alert>
    </div>
</template>

<!-- eslint-disable vue/multi-word-component-names -->
<script>
import PlayerCard from '@/components/troefcall/game/PlayerCard.vue'
import Card from '@/components/troefcall/game/Card.vue'

export default {
    name: "RoundTable",
    props: {
        round: {
            type: Object,
            required: true,
        },
        players: {
            type: Array,
            required: true,
        },
        compact: {
            type: Boolean,
            required: false,
            default: false,
        },
    },
    data(){
        return {
            cards: [],
        }
    },
    components: {
    Card,
    PlayerCard
},
    mounted() {
        console.log('Mounted table: ' + JSON.stringify(this.round, null, 2));
    },
    methods:{
        getPlayerName(index) {
            index = (index + this.round.player_starting) % 4;
            if (this.players == null || this.players == undefined) {
                console.error('Players is null or undefined');
                return '';
            }
            return this.players[index].name;
        },
        getCard(index){
            return this.round.played_cards[index];
        }
    },
}
</script>