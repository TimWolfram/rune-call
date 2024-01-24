<template>
    <v-card>
        <p v-if="pref.preferSimpleCard === true">
            <!-- show card suit and value with unicode -->
            {{ card.suit }} {{ card.value }}
        </p>
        <p v-else>
            <!-- show error: NYI -->
            {{ getCardUnicode }}
        </p>
    </v-card>
</template>

<script>
import { ref, onMounted } from 'vue';
import { usePreferencesStore } from '@/store/preferences';

export default {
    name: "Card",
    props: {
        card: Object,
    },
    setup(props) {
        const pref = usePreferencesStore();
        return {
            pref,
        }
    },
    data() {
        return {
            card: this.card,
        }
    },
    methods: {
        getCardUnicode() {
            //TODO: implement
        },
    },
    mounted() {
        console.log('Mounted card: ' + JSON.stringify(this.card, null, 2));
    },
}
</script>