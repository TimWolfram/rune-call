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
    <v-btn color="primary" size="large" @click="login(username, password)">Login</v-btn>
  </v-container>
  <v-container v-else>
    <!-- user is logged in --->
    <p class="text-h5">Welcome, {{ loginUser.nickname }}!</p>
    <br/>
    <!-- display create room and browse rooms buttons -->
    <v-btn block color="primary" size="large" @click="createRoom">Create Room</v-btn> <br/>
    <v-btn block color="primary" size="large" @click="browseRooms">Browse Rooms</v-btn> <br/>
    <v-btn block color="primary" size="large" @click="logOut">Log out</v-btn> <br/>
  </v-container>
</template>

<script>
  import { get, del } from '@/requests'
  
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
        this.$router.push('/createroom');
      },
      browseRooms() {
        // redirect to room browse screen
        this.$router.push('/rooms');
      },
      async logOut() {
        // redirect to room browse screen
        let response = await del('login');
        if (!response.status === 200) {
          console.log('Server response was not ok: ' + response.status + '\n' + response.statusText);
        }
        else {
          console.log('logged out');
        }
        this.$router.push('/');
      },
      async update() {
        // let pageUrl = 'login/testadmin'; 
        let pageUrl = 'login'; //login without form will check current logged in user
        get(pageUrl,{}).then((response) =>{
          let loginResponse = response.data;
          //print rooms to console as json
          console.log(`logged in user: \n${JSON.stringify(loginResponse, null, 2)}`);
          console.log('cookies from login response: ' + JSON.stringify(response.headers, null, 2));
          this.loginErr = false; // API call was successful
          if (loginResponse.user_id !== null) {
            this.loginUser = loginResponse;
          }
        })
        .catch((error) => {
          console.log('Server response was not ok: ' + error);
          this.loginErr = true; // API call failed
        });
      },
    },
    mounted() {
      console.log(`the home component is now mounted.`)
      this.update();
    }
  }
</script>