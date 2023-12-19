<template>
  <v-col>
    <v-card
      class="pa-1 ma-1 text-center"
      height="200"
      width="200"
      rounded="lg"
      @click="handleClick(room.id)">
      <v-card-title>{{ getRoomDisplayName(room.name) }} <v-icon v-if="room.password !== ''"><br/>mdi-lock</v-icon></v-card-title>
      <v-card-text>
          Host: {{ getHostName(room.host_id) }}
          <br/>
          Players: {{ getActivePlayersCount(room.players) }}/4
        </v-card-text>
    </v-card>
  </v-col>
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
    },
    getRoomDisplayName(roomName) {
      roomName = roomName ? roomName : 'Unnamed room';
      // limit room name to 20 characters
      return roomName.length > 20 ? roomName.substring(0, 20) + '...' : roomName;
    },
    getActivePlayersCount(players) {
      return players.filter(player => player && player.user_id !== null).length;
    },
    handleClick() {
      console.log('Card clicked!');
      // redirect to room screen
      this.$router.push('/rooms/' + this.room.id);
    }
  }
}
</script>
