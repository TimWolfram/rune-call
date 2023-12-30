<template>
  <v-card
            class="pa-1 ma-1 text-center"
            width="200"
            height="200"
            rounded="lg"
            @click="handleClick(room.id)"> 
    <v-card-title>
      {{ getRoomDisplayName(room.name) }} <v-icon v-if="room.password !== ''"><br/>mdi-lock</v-icon>
    </v-card-title> 
    <v-card-text>
      Host: {{ getHostName(room.host_id) }}
      <br/>
      Players: {{ getActivePlayersCount(room.players) }}/4
    </v-card-text>
  </v-card>
</template>

<script>
import { VCard, VCardTitle, VCardText, VIcon } from 'vuetify/components';
import { getShortName } from '@/utils';
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
      return getShortName(host ? host.name : 'Unknown', 20);
    },
    getRoomDisplayName(roomName) {
      roomName = roomName ? roomName : 'Unnamed room';
      // limit room name to 13 characters
      return getShortName(roomName, 13);
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