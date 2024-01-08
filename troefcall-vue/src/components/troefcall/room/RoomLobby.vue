<template>
  <v-app>
    <v-container v-if="error === null">
      <v-alert type="info">Loading...</v-alert>
    </v-container>
    <v-container v-else-if="error === true">
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
        <v-btn class="ma-1" color="error" @click="leave">Leave room</v-btn> 
        <v-btn class="ma-1" color="success" @click="start" v-if="isHost">Start game</v-btn>
      </div>      
    </v-container>
  </v-app>
</template>
  
<script setup>
  import { onMounted } from 'vue';
  import { ref } from 'vue';
  import user from '@/components/troefcall/room/RoomUser.vue';
  import { get } from '@/requests';
    const props = defineProps( {
        id: {
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
  const error = ref(null);
  const room = ref(null);
  const isHost = ref(true);
  onMounted(() => {
    console.log('Mounted room item');
    get('rooms/' + props.id).then(response => {
      console.log('Room data: ' + JSON.stringify(response.data));
      room.value = response.data;
      error.value = false;
    }).catch(error => {
      console.error('Failed to fetch room data: ' + error);
      error.value = true;
    });
  });
  function join(index) {
    console.warn(`TODO: Joining at seat ${index}`);
  }
  function leave() {
    console.warn('TODO: leave room');
  }
  function start() {
    console.warn('TODO: start game');
  }
</script>