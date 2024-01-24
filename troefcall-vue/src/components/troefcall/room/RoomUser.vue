<template>
  <v-card class="d-flex ma-1 align-center justify-center" 
    width="150" height="100" 
    rounded="lg">

    <v-container v-if="user" class="align-content-center text-center pa-0">
      <!-- Display user information here -->
      <v-card-text class="text-body-1 pa-0 ma-0">{{ user.name }}</v-card-text>
    </v-container>
    <v-btn block min-height="100%" min-width="100%" v-else-if="auth.loggedIn" @click="$emit('join')" text="Join" />
    <router-link to="/login" v-else>Log in to join</router-link>
  </v-card>
</template>
  
<script setup>
import { useAuthStore } from '@/store/auth';
import { onMounted, ref } from 'vue';

defineProps({
  user: Object
});
const auth = useAuthStore();

onMounted(() => {
  console.log('Mounted user card');
});
</script>