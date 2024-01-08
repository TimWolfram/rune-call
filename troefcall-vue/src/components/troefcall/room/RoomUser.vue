<template>
  <v-card
        class="d-flex pa-1 ma-1 align-center justify-center"
        width="150"
        height="100"
        min-width="150"
        min-height="100"
        rounded="lg">
    
    <div v-if="user"
        class="align-content-center text-center">
    <!-- Display user information here -->
    <v-card-title>{{ user.name }}</v-card-title>
    </div>
    <div v-else>
      <div v-if="auth.loggedIn">
        <v-btn @click="join">Join</v-btn>
      </div>
      <router-link to="createroom" v-else>Log in to join</router-link>
    </div>
  </v-card>
</template>
  
<script setup>
  import { useAuthStore } from '@/store/auth';
  import { onMounted } from 'vue';
  defineProps({
    user: Object
  });
  const auth = useAuthStore();
  function join() {
    // emit join event
    this.$emit('join');
    
  }
  onMounted(() => {
    console.log('Mounted user card');
  });
</script>