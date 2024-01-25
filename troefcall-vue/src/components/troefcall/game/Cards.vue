<template>
    <v-card class="d-flex align-center justify-center ma-1 pa-1" width="300" height="150" rounded="lg">
        <v-card-text class="text-body-1">Cards</v-card-text>
        <v-card v-for="card in cards" class="d-flex align-center justify-center">
            <v-card class="ma-1 pa-1" width="50" height="100" rounded="lg">
                <v-card-text class="text-body-1">{{ card }}</v-card-text>
            </v-card>
        </v-card>
    </v-card>
</template>

<script>
import { ref, onMounted } from 'vue';
import { usePreferencesStore } from '@/store/preferences';
export default {
    name: "Cards",
    props: {
        cards: Array,
    },
    mounted() {
        console.log('Mounted card: ' + JSON.stringify(this.card, null, 2));
    },
}
</script>