<template>
  <v-container>
    <div v-if="rooms.length === 0">
      <p>No rooms available! Create a new room or try again later.</p>
      <!-- link to room create screen -->
      <router-link to="createroom">Create a new room</router-link>
    </div>
    <div v-else>
      <v-row>
        <RoomListItem v-for="room in rooms" :key="room.id" :room="room"/>
      </v-row>
    </div>
  </v-container>
</template>

<script>
  import RoomListItem from '@/components/troefcall/RoomListItem.vue';
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
