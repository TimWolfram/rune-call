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

<script setup>
  import RoomListItem from '@/components/troefcall/RoomListItem.vue';
  import { ref, onMounted } from 'vue';
  import { HOST } from '../constants';

  let rooms = ref([]); // reactive
  onMounted(async () => {
    const response = await fetch(`${HOST}/rooms`);
    rooms.value = await response.json();
  });
</script>
