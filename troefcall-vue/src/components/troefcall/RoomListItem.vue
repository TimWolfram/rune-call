<template>
  <v-card
    v-click-outside="onClickOutside"
    :color="active ? 'primary' : undefined"
    :dark="active"
    class="flex-lg-grow-1 ma-2"
    rounded="xl">
    <v-card-title>{{ room.name }} <v-icon v-if="room.password !== ''"><br/>mdi-lock</v-icon></v-card-title>
    <v-card-text>
      Host: {{ getHostName(room.host_id) }}
      <br/>
      Players: {{ getActivePlayersCount(room.players) }}/4
    </v-card-text>
  </v-card>
</template>

<script>
import { VCard, VCardTitle, VCardText, VIcon } from 'vuetify/components';

export default {
  name: "RoomListItem",
  components: {
    VCard,
    VCardTitle,
    VCardText,
    VIcon
  },
  props: {
    room: Object
  },
  methods: {
    getHostName(hostId) {
      const host = this.room.players.find(player => player && player.user_id === hostId);
      return host ? host.name : 'Unknown';
    }
  }
}
</script>
