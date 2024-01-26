<template>
    <v-card width="400"
        color="#005500"
        class="ma-1 pa-1"
        rounded="lg"
    >   
        <v-container class="d-flex align-content-center justify-center">
            <Card :card="round.played_cards[0]"/>
        </v-container>
        <v-container class="d-flex align-content-center">
            <v-spacer/>
            <Card :card="round.played_cards[3]"/>
            <v-spacer/>
            <Card :card="round.played_cards[1]"/>
            <v-spacer/>
        </v-container>
        <v-container class="d-flex align-content-center justify-center">
            <Card :card="round.played_cards[2]"/>
        </v-container>
    </v-card>
</template>

<!-- eslint-disable vue/multi-word-component-names -->
<script>
import Card from '@/components/troefcall/game/Card.vue';
export default {
    name: "RoundTable",
    props: {
        round: {
            type: Object,
            required: true,
        },
    },
    data(){
        return {
            cards: [],
        }
    },
    components: {
        Card,
    },
    mounted() {
        console.log('Mounted table: ' + JSON.stringify(this.round, null, 2));
    },
    // methods:{
    //     getCards() {
    //         if (this.round === null || this.round === undefined) {
    //             console.error('Round is null or undefined');
    //             return;
    //         }
    //         this.cards = this.round.played_cards;
    //     },
    // },  
}
</script>