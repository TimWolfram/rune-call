<template>
    <div v-if="rooms.length === 0">
      <p>No rooms available! Create a new room or try again later.</p>
      <!-- link to room create screen -->
      <router-link to="rooms/create">Create a new room</router-link>
    </div>
    <v-container v-else 
    class="d-flex flex-wrap"
  >
  <RoomListItem 
    class="d-flex align-content-space-around flex-wrap bg-surface-variant"
        :room="room"
        v-for="room in rooms" :key="room.id" />        
    <!-- <v-sheet v-for="n in 20" :key="n" class="ma-2 pa-2"> Flex item </v-sheet> -->
  </v-container>
</template>

<script>
  import RoomListItem from '@/components/troefcall/room/RoomListItem.vue';
  export default {
    name: "RoomList",
    components: {
      RoomListItem
    },
    props: {
      rooms: Array
    },
    methods: {
      clickRoom(roomId) {
        console.log('Room clicked!');
        // redirect to room screen
        this.$router.push('rooms/' + roomId);
      }
    }
  }
</script>
