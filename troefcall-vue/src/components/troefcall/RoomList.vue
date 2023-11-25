<template>
  <div class="d-flex flex-wrap justify-space-between">
    <div v-if = "rooms.length === 0">
      <p>No rooms available! Create a new room or try again later.</p>
      <!-- link to room create screen -->
      <router-link to="/rooms/create">Create a new room</router-link>
    </div>

    <RoomListItem v-for="room in rooms" :key="room.id" :room="room" @click="active = true"/>
  </div>
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
    }
  }
</script>
