<template>
    <v-card width="300" class="ma-1 pa-1" rounded="lg">
        <v-container class="d-flex flex-column align-content-center justify-space-evenly">
            <div v-if="false">
                <v-card v-for="(player, index) in getScores" :key="index"
                class="d-flex flex-row align-content-center justify-space-between ma-1" width="200" height="50"
                rounded="lg">
                    <v-card-text class="text-body-2">{{ player.name }}</v-card-text>
                    <v-card color="background">
                        <v-card-text class="text-body-2">{{ player.score }}</v-card-text>
                    </v-card>
                    <v-card color="#002200">
                        <v-card-text class="text-body-2">{{ player.totalScore }}</v-card-text>
                    </v-card>
                </v-card>
            </div>
            <div v-else>
                <v-table density="compact">
                    <thead>
                        <tr>
                            <th class="text-left">
                                Player
                            </th>
                            <th class="text-left text-truncate">
                                <v-tooltip text="Total rounds won">
                                <template v-slot:activator="{ props }">
                                    <p v-bind="props">W</p>
                                </template>
                                </v-tooltip>
                            </th>
                            <th class="text-left">
                                <v-tooltip text="Total score for team">
                                <template v-slot:activator="{ props }">
                                    <p v-bind="props">S</p>
                                </template>
                                </v-tooltip>
                            </th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr
                        v-for="(player, index) in getScores"
                        :key="index"
                        >
                        <td class="text-truncate">{{ player.name }}</td>
                        <td>{{ player.score }}</td>
                        <td>{{ player.totalScore }}</td>
                        <!-- <div :color="index % 2 == 0? '#FF0000' : '#00FF00'" class="d-flex flex-row align-content-center justify-center ">
                        </div> -->
                    </tr>
                    </tbody>
                </v-table>
            </div>
        </v-container>
    </v-card>
</template>

<!-- eslint-disable vue/multi-word-component-names -->
<script>
export default {
    name: "ScoreDisplay",
    props: {
        game: {
            type: Object,
            required: true,
        },
    },
    computed: {
        getScores() {
            if (this.game === null || this.game === undefined) {
                console.error('Game is null or undefined');
                return;
            }
            let players = this.game.players;
            if (players === null || players.length === 0) {
                console.error('Players is null or empty');
                return;
            }
            for (let i = 0; i < players?.length; i++) {
                players[i].score = 0;
                players[i].totalScore = 0;
                players[i].roundsWon = [];
            }

            //calculate scores for each player
            const played_rounds = this.game.played_rounds;
            for (let i = 0; i < played_rounds?.length; i++) {
                let round = played_rounds[i];
                const winnerIndex = round.state.RoundWon.winner_user_index % 4;
                if (winnerIndex === null || winnerIndex === undefined) {
                    console.error('Winner index is null or undefined\n' + JSON.stringify(round, null, 2));
                    return;
                }
                const teammateIndex = (winnerIndex + 2) % 4;
                
                players[winnerIndex].roundsWon.push(round);
                players[winnerIndex].score += 1;
                players[winnerIndex].totalScore += 1;
                players[teammateIndex].totalScore += 1;
            }
            let header = {};
            header.name = 'Player';
            header.score = 'Score';
            header.totalScore = 'Total';
            return players;
        },
    },
    methods: {
    },
}
</script>