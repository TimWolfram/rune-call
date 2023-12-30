<template>
    <div v-for="(user, index) in users" :key="index">
        <div v-if="user">
        <!-- Display user information here -->
        <v-card>{{ user.name }}</v-card>
        </div>
        <div v-else>
        <!-- Display join button if user is null -->
        <v-card @click="join(index)">Join</v-card>
        </div>
    </div>
</template>
  
<script setup>
  import { onMounted } from 'vue';
  import { ref } from 'vue';
  
  // Sample users data
  const users = ref([
    { name: 'User 1' },
    null,
    { name: 'User 2' },
    null,
    // Add more users as needed
  ]);
  onMounted(() => {
    console.log('Mounted room item');
  });
  function join(index) {
    // Implement your join logic here
    console.log(`Joining at seat ${index}`);
    //TODO join room
  }
</script>