<template>
  <v-container v-if="!auth.loggedIn">
    <p class="text-h5">Welcome to Tjall!</p>
    <br/>
    <p>
      Tjall is a chat application for playing Tjall (a.k.a "Troefcall").
      <br/>
      You can create a room and invite your friends to play with you. <router-link to="about">Click here</router-link> to read more about Tjall.
      <br/> <br/>
      To get started, <router-link to="login">log in</router-link> or <router-link to="register"> register an account </router-link>.
    </p>
  </v-container>
  <v-container v-else> <!-- user is logged in --->
    <p class="text-h5">Welcome, {{ auth.getDisplayName }}!</p>
    <br/>
    <p>
      You are logged in as {{ auth.getUsername }}. You can change your display name in the settings page.
      <br/>
      You can create a room and invite your friends to play with you. <router-link to="about">Click here</router-link> to read more about Tjall.
    </p>
    <br/>
    
    <!-- display create room and browse rooms buttons -->
    <v-btn block color="primary" size="large" to="rooms/create">Create Room</v-btn> <br/>
    <v-btn block color="primary" size="large" to="rooms">Browse Rooms</v-btn> <br/>
    <br/>
    <v-btn block color="error" size="large" @click="auth.logout">Log out</v-btn> <br/>
    
  </v-container>
</template>

<!-- eslint-disable vue/multi-word-component-names -->
<script>
  import { useAuthStore } from '@/store/auth'
  export default {
    name: "Home",
    
    data() {
      return {
        rooms: [],
        page: 0,
      auth: useAuthStore(),
      }
    },
  }
</script>