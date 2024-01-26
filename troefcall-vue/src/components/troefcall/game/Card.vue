<template>
    <v-card v-if="pref.preferSimpleCard === true" color="#FFFFFF" class="ma-1 pa-1" width="50" height="75" rounded="lg">
        <div :style="{ color: getColor() }" >
            <p class="text-h5">{{ unicode.substring(0, 1) }} </p>
            <p class="text-h5">{{ unicode.substring(1) }} </p>
        </div>
    </v-card>
    <div v-else>
        <!-- TODO: fancier card display -->
        {{ card.value }} of {{ card.suit }}
    </div>
</template>

<!-- eslint-disable vue/multi-word-component-names -->
<script>
import { ref } from 'vue';
import { usePreferencesStore } from '@/store/preferences';

export default {
    name: "Card",
    props: {
        card: Object,
    },
    data() {
        return {
            pref: usePreferencesStore(),
            unicode: ref(''),
            color: 'black',
        }
    },
    methods: {
        getColor() {
            //determine suit by first letter
            let suit = this.card?.suit.charAt(0);

            if (suit === 'H' || suit === 'D') {
                return '#dd0000';
            }
            return '#000000';
        },
        getCardUnicode() {
            //determine suit by first letter
            let suit = this.card?.suit.charAt(0);
            if (suit === 'H') {
                suit = '♥';
            }
            else if (suit === 'D') {
                suit = '♦';
            }
            else if (suit === 'S') {
                suit = '♠';
            }
            else if (suit === 'C') {
                suit = '♣';
            }
            else {
                suit = '♠';
            }
            let value = this.card?.value;
            switch (value) {
                case 11:
                    value = 'J';
                    break;
                case 12:
                    value = 'Q';
                    break;
                case 13:
                    value = 'K';
                    break;
                case 14:
                    value = 'A';
                    break;
                default:
                    value = value.toString();
                    break;
            }
            return suit + value;
        },
    },
    mounted() {
        this.unicode = this.getCardUnicode();
    },
}
</script>