<!-- eslint-disable vue/multi-word-component-names -->
<template>
  <v-container v-if="!auth.isLoggedIn">
    <p class="text-h5">Welcome to Tjall!</p>
    <br/>
    <p>
      Tjall is a chat application for playing Tjall (a.k.a "Troefcall"). You can create a room
      and invite your friends to play with you. Read more about Tjall <a href="https://en.wikipedia.org/wiki/Troefcall">here</a>.
    </p>
    <br/>
    <p>
      To get started, <router-link to="login">log in</router-link> or register an account.
    </p>
  </v-container>
  <v-container v-else>
    <!-- user is logged in --->
    <p class="text-h5">Welcome, {{ auth.getDisplayName }}!</p>
    <br/>
    <!-- display create room and browse rooms buttons -->
    <v-btn block color="primary" size="large" @click="createRoom">Create Room</v-btn> <br/>
    <v-btn block color="primary" size="large" @click="browseRooms">Browse Rooms</v-btn> <br/>
    <v-btn block color="primary" size="large" @click="logOut">Log out</v-btn> <br/>
  </v-container>
</template>

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
    methods: {
      createRoom() {
        // redirect to room create screen
        this.$router.push('/createroom');
      },
      browseRooms() {
        // redirect to room browse screen
        this.$router.push('/rooms');
      },
    },
  }
</script>