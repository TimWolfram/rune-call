<template>
  <v-card>
    <v-container v-if="roomDataError === null">
      <v-alert type="info">Loading...</v-alert>
    </v-container>

    <v-container v-else-if="roomDataError === true">
      <v-alert type="error">Could not reach server; try again later!</v-alert>
    </v-container>

    <v-container v-else-if="roomDataError === 'gone'">
      <v-alert type="error">This room is gone!</v-alert>
    </v-container>
    <v-container v-else-if="roomDataError === 'NaN'">
      <v-alert type="error">Room id is not a number!</v-alert>
    </v-container>

    <v-container v-else>
      <div>
        <v-card-title >{{room.name}}</v-card-title>
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
          v-if=isPlayer()
          class="ma-1"
          color="error"
          @click="leave">Leave room</v-btn> 
        <v-btn
          v-if="canStart()" 
          class="ma-1"
          color="success"
          @click="startGame">Start game</v-btn>
      </div>      
    </v-container>
  </v-card>
</template>
  
<script setup>
  import { onMounted, onBeforeUnmount } from 'vue';
  import { ref } from 'vue';
  import { useRouter } from 'vue-router';
  import { get, post, del } from '@/requests';
  import user from '@/components/troefcall/room/RoomUser.vue';
  import { useAuthStore } from '@/store/auth';
  import { LOBBY_REFRESH_INTERVAL } from '@/store/preferences';
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
  const roomDataError = ref(null);
  const room = ref(null);
  const auth = useAuthStore();
  const router = useRouter();

  let refresher = null;
  
  onMounted(() => {
    console.log('Mounted room lobby');
    //check if room id is a number
    if (isNaN(props.roomId)) {
      console.error('Room id is not a number: ' + props.roomId);
      roomDataError.value = "NaN";
      return;
    }
    refresher = setInterval(refresh, LOBBY_REFRESH_INTERVAL);
    refresh();
  });
  onBeforeUnmount(() => {
    console.log('Unmounted room lobby');
    clearInterval(refresher);
  });

  function refresh(){
    getRoomData();
    //redirect if game is in progress
    if(room?.value?.game_in_progress){
      console.log('Game is in progress, redirecting to game');
      router.push('/rooms/' + props.roomId + '/game');
    }
  }
  
  function getRoomData(){
    get('rooms/' + props.roomId)
    .then(response => {
      console.log('Room data: ' + JSON.stringify(response.data));
      room.value = response.data;
      roomDataError.value = false;
    }).catch(error => {
      //check if 410 (gone)
      if(error.response.status === 410){
        console.log('Room ' + props.roomId + ' is gone');
        roomDataError.value = "gone";
        return;
      }
      console.error('Failed to fetch room data: ' + error);
      roomDataError.value = true;
    });
  }

  function join(index) {
    post('rooms/' + props.roomId + '/players/' + index)
      .then(response => {
        console.log('Joined room: ' + JSON.stringify(response.data));
        room.value = response.data;
        auth.setCurrentRoom(props.roomId);
      }).catch(error => {
        console.error('Failed to join room: ' + error.response.data);
      });
  }

  function leave() {
    if (!isPlayer()) {
      console.error('Not a player, cannot leave room');
      return;
    }
    let id = useAuthStore().user.id;
    del('rooms/' + props.roomId + '/players/')
      .then( () => {
        console.log(`Player #${id} left room #${props.roomId}`);
        room.value.players[id] = null;
        let user = useAuthStore().user;
        user.current_room = null;
        useAuthStore().setUser(user);
        getRoomData();
      }).catch(error => {
        console.error('Failed to leave room: ' + error.response.data);
      });
  }

  function startGame() {
    post('rooms/' + props.roomId + '/game')
      .then(response => {
        console.log('Started game: ' + JSON.stringify(response.data));
        router.push('/rooms/' + props.roomId + '/game');
      }).catch(error => {
        console.error('Failed to start game: ' + error.response.data);
      });
  }
  function canStart() {
    if(!isHost()){
      return false;
    }
    for(let i = 0; i < room.value.players.length; i++){
      if(room.value.players[i] == null){
        return false;
      }
    }
    return true;
  }
  function isHost() {
    let user = auth?.user;
    if (!user) {
      console.error('Not logged in, cannot check if user is host');
      return false;
    }
    return room.value.host_id === user?.id;
  }
  
  function isPlayer() {
    let user = auth?.user;
    if (!user) {
      console.error('Not logged in, cannot check if user is player');
      return false;
    }
    if(user.current_room != null & user.current_room === room.value.id){

      console.log('Is player from current room (' + user.current_room + ')');
      return true;
    }
    const id = user?.id ?? -1;
    console.log('User id: ' + id + ', room: ' + JSON.stringify(room.value));
    for (let i = 0; i < room.value.players.length; i++) {
      if (room.value.players[i]?.id === id) {
        console.log('Is player: ' + true);
        return true;
      }
    }
    console.log('Is player: ' + false);
    return false;
  }
</script>