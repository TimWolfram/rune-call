<template>
    <v-card color="background" class="d-flex flex-column align-center justify-center ma-1 pa-1 bg-red" rounded="lg">
        <v-card-text class="text-body-1">Cards</v-card-text>
        <v-container class="d-flex flex-wrap align-center justify-center">
            <Card 
                v-for="(card, index) in cards" 
                :key="index" 
                :card="card" @click="onClick(card)" 
                :style="getCardStyle(card)"
                />
        </v-container>
        <v-container width="300" class="d-flex flex-column align-center justify-center ma-1 pa-1" v-if="selected">
            <v-card-text class="text-body-1">Selected: {{ getCardText() }}</v-card-text>
            <v-container class="d-flex align-center justify-center">
                <v-btn width="100" class="ma-1" color="error" @click="selected = null">Clear</v-btn>    
                <v-btn width="100" class="ma-1" color="success" @click="playCard">Confirm</v-btn>
            </v-container>
        </v-container>
    </v-card>
</template>

<!-- eslint-disable vue/multi-word-component-names -->
<script>
import Card from '@/components/troefcall/game/Card.vue';
export default {
    name: "Cards",
    props: {
        cards: Array,
    },
    data() {
        return {
            selected: null,
        }
    },
    components: {
        Card,
    },
    methods: {
        onClick(card) {
            this.selected = card;
        },
        playCard() {
            console.log('TODO - Playing card: ' + JSON.stringify(this.selected, null, 2));
            this.selected = null;
        },
        getCardText() {
            if(this.selected == null) {
                return 'NONE';
            }
            let value = this.selected.value;
            switch(value) {
                case 11:
                    value = 'Jack';
                    break;
                case 12:
                    value = 'Queen';
                    break;
                case 13:
                    value = 'King';
                    break;
                case 14:
                    value = 'Ace';
                    break;
                default:
                    break;
            }
            return value + ' of ' + this.selected.suit;
        },
        getCardStyle(card) {
            if (this.selected?.suit == card.suit && this.selected?.value == card.value) {
                return {
                    'border': '2pt solid #DD0000',
                }
            }
            else {
                return {
                    'border': '2pt solid #000000',
                }
            }
        },
    },
}
</script>