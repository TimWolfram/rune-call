<!-- eslint-disable vue/multi-word-component-names -->
<template>
  <v-container v-if="loginErr === null">
    <v-alert type="info">Loading...</v-alert>
  </v-container>
  <v-container v-else-if="loginErr === true">
    <v-alert type="error">Could not reach server; try again later!</v-alert>
  </v-container>
  <v-container v-else-if="loginUser === null">
    <!-- user is not logged in --->
    <p>Log in</p>
    <!-- display login (username, pw, button) -->
    <v-text-field v-model="username" label="Username" />
    <v-text-field v-model="password" label="Password" />
    <v-btn color="primary" @click="login(username, password)">Login</v-btn>
  </v-container>
  <v-container v-else>
    <!-- user is logged in --->
    <p>Logged in as {{ login }}</p>
    <!-- display create room and browse rooms buttons -->
    <v-btn color="primary" @click="createRoom">Create Room</v-btn>
    <v-btn color="primary" @click="browseRooms">Browse Rooms</v-btn>
  </v-container>
</template>

<script>
  import { ref, onMounted } from 'vue';
  import HOST from '@/constants'
  import axios from 'axios'

  export default {
    name: "Home",
    
    data() {
      return {
        rooms: [],
        page: 0,
        loginErr: null,
        loginUser: null
      }
    },
    methods: {
      createRoom() {
        // redirect to room create screen
        this.$router.push('/rooms/create');
      },
      browseRooms() {
        // redirect to room browse screen
        this.$router.push('/rooms');
      },
      async update() {
        let pageUrl = HOST + '/login'; //login without form will check current logged in user
        try {
          let response = await fetch(pageUrl);
          if (!response.ok) {
            throw new Error('Server response was not ok');
          }
          let loginResponse = await response.json();
          //print rooms to console as json
          console.log("logged in user: \n" + JSON.stringify(loginResponse));
          this.loginErr = false; // API call was successful
          if (loginResponse.user_id !== null) {
            this.loginUser = loginResponse;
          }
        } catch (login) {
          console.error('Failed to fetch rooms:', login);
          this.loginErr = true; // API call failed
        }
      },
      onMounted() {
        console.log(`the home component is now mounted.`)
        this.update();
      }
    }
  }
</script>