<template>
    <v-card class="d-flex flex-column align-center justify-center ma-1 pa-1 bg-red" rounded="lg">
        <v-card-text class="text-body-1">Your cards:</v-card-text>
        <div class="d-flex flex-wrap align-center justify-center">
            <Card 
                v-for="(card, index) in cards" 
                :key="index" 
                :card="card" @click="onClick(card)" 
                :style="getCardStyle(card)"
                />
        </div>
        <div width="300" class="d-flex flex-column align-center justify-center ma-1 pa-1" v-if="selected">
            <v-card-text class="text-body-1">Selected: {{ getCardText() }}</v-card-text>
            <v-container class="d-flex align-center justify-center">
                <v-btn width="100" class="ma-1" color="error" @click="clearSelect">Clear</v-btn>    
                <v-btn width="100" class="ma-1" color="success" @click="playCard">Confirm</v-btn>
            </v-container>
        </div>
    </v-card>
</template>

<!-- eslint-disable vue/multi-word-component-names -->
<script>
import Card from '@/components/troefcall/game/Card.vue';
export default {
    name: "Cards",
    props: {
        cards: Array,
        tjall: String,
    },
    data() {
        return {
            selected: null,
        }
    },
    components: {
        Card,
    },
    mounted() {
        console.log('Mounted cards: ' + JSON.stringify(this.cards, null, 2));
    },
    methods: {
        onClick(card) {
            this.selected = card;
            this.$emit('onSelect');
        },
        clearSelect() {
            this.selected = null;
            this.$emit('onSelect');
        },

        playCard() {
            console.log('Playing card: ' + JSON.stringify(this.selected, null, 2));
            this.$emit('onPlayCard', this.selected);
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
            if(card.suit == this.tjall) {
                return {
                    'border': '2pt solid #00DD00',
                }
            }
            return {
                'border': '2pt solid #000000',
            }
        },
    },
}
</script>