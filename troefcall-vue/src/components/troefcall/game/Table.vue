<template>
    <v-card class="ma-1 pa-1" width="90%" height="300" rounded="lg">
        <v-card-text class="text-body-1 pa-0 ma-0">Table</v-card-text>
    </v-card>
  </template>

<script>
import { ref, onMounted } from 'vue';
import { usePreferencesStore } from '@/store/preferences';

export default {
    name: "Table",
    props: {
        game: Object,
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
    
    },
    mounted() {
        console.log('Mounted card: ' + JSON.stringify(this.card, null, 2));
    },
}
</script>