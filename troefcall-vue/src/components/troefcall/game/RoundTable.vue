<template>
    <v-card class="ma-1 pa-1" width="90%" height="300" rounded="lg">
        <v-card-text class="text-body-1 pa-0 ma-0">Table</v-card-text>
    </v-card>
</template>

<!-- eslint-disable vue/multi-word-component-names -->
<script>
export default {
    name: "RoundTable",
    props: {
        game: Object,
    },
    mounted() {
        console.log('Mounted table: ' + JSON.stringify(this.game, null, 2));
    },
}
</script>