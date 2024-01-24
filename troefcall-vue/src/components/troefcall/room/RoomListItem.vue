<template>
  <v-card
            class="pa-2 ma-2 text-center"
            width="300"
            height="200"
            rounded="lg"
            @click="handleClick(room.id)"> 
    <v-card-title>
      <div class="d-flex justify-space-between"> 
        {{ getRoomDisplayName(room.name) }}
        <v-spacer/>
        <v-icon v-if="room.password !== ''"><br/>mdi-lock</v-icon>
        <v-icon v-else><br/>mdi-lock-open-outline</v-icon>
      </div>
    </v-card-title> 
    <v-divider/>
    <v-card-text>
      Host: {{ getHostName(room.host_id) }}<br/>
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