<template>
  <v-container v-if="!auth.isLoggedIn">
    <p class="text-h5">Welcome to Tjall!</p>
    <br/>
    <p>
      Tjall is a chat application for playing Tjall (a.k.a "Troefcall").
      <br/>
      You can create a room and invite your friends to play with you. Read more about Tjall <router-link to="about">here</router-link>.
      <br/> <br/>
      To get started, <router-link to="login">log in</router-link> or register an account.
    </p>
  </v-container>
  <v-container v-else>
    <!-- user is logged in --->
    <p class="text-h5">Welcome, {{ auth.getDisplayName }}!</p>
    <br/>
    <!-- display create room and browse rooms buttons -->
    <v-btn block color="primary" size="large" to="createroom">Create Room</v-btn> <br/>
    <v-btn block color="primary" size="large" to="rooms">Browse Rooms</v-btn> <br/>
    <v-btn block color="primary" size="large" @click="logOut">Log out</v-btn> <br/>
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