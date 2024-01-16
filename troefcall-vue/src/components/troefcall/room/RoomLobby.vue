<template>
  <v-app>
    <v-container v-if="getRoomDataError === null">
      <v-alert type="info">Loading...</v-alert>
    </v-container>

    <v-container v-else-if="getRoomDataError === true">
      <v-alert type="error">Could not reach server; try again later!</v-alert>
    </v-container>

    <v-container v-else>
      <div class="d-flex align-content-center justify-center">
        <v-card-title>
          {{ room.name}}
        </v-card-title>
      </div>
      <div class="d-flex align-content-center justify-center">
        <user :user="room.players[0]" v-on:join="join(0)"/>
      </div>
      <div class="d-flex justify-space-evenly">
        <user :user="room.players[1]" v-on:join="join(1)"/>
        <user :user="room.players[3]" v-on:join="join(3)"/>
      </div>
      <div class="d-flex align-content-center justify-center">
        <user :user="room.players[2]" v-on:join="join(2)"/>
      </div>
      <div class="d-flex align-content-center justify-center">
        <v-btn
          v-if="isPlayer"
          class="ma-1"
          color="error"
          @click="leave">Leave room</v-btn> 
        <v-btn
          v-if="isHost" 
          class="ma-1"
          color="success"
          @click="startGame">Start game</v-btn>
      </div>      
    </v-container>
  </v-app>
</template>
  
<script setup>
  import { onMounted } from 'vue';
  import { ref } from 'vue';
  import { get, post, del } from '@/requests';
  import user from '@/components/troefcall/room/RoomUser.vue';
  import { useAuthStore } from '@/store/auth';

  const props = defineProps({
    roomId: {
      type: Number,
      required: true,
    }
  });
  // Sample users data
  // const users = ref([
  //   { name: 'User 1' },
  //   null,
  //   { name: 'User 2' },
  //   null,
  // ]);
  const getRoomDataError = ref(null);
  const room = ref(null);

  onMounted(() => {
    console.log('Mounted room item');
    getRoomData();
  });
  
  function getRoomData(){
    get('rooms/' + props.roomId).then(response => {
      console.log('Room data: ' + JSON.stringify(response.data));
      room.value = response.data;
      getRoomDataError.value = false;
    }).catch(error => {
      console.error('Failed to fetch room data: ' + error);
      error.value = true;
    });
  }
  function join(index) {
    console.warn(`TODO: Joining at seat ${index}`);
    post('rooms/' + props.roomId + '/players/' + index).then(response => {
      console.log('Joined room: ' + JSON.stringify(response.data));
      room.value = response.data;
    }).catch(error => {
      console.error('Failed to join room: ' + error);
    });
  }

  function leave() {
    if (!isPlayer()) {
      console.error('Not a player, cannot leave room');
      return;
    }
    let id = useAuthStore().user.id;
    del('rooms/' + props.roomId + '/players/' + id).then( () => {
      console.log(`Player #${id} left room #${props.roomId}`);
    }).catch(error => {
      console.error('Failed to leave room: ' + error);
    });
  }
  function startGame() {
    console.warn('TODO: Start game');
  }
  function isHost() {
    return room.value.host.id === useAuthStore().user.id;
  }
  function isPlayer() {
    return room.value.players.some(player => player.id === useAuthStore().user.id);
  }
</script>