<template>
  <v-container>
    <!-- logged in -->
    <div v-if="auth.loggedIn == false">
      You must <RouterLink to="/login">log in</RouterLink> before creating a room.
    </div>
    <!-- already in room -->
    <div v-else>
      <div v-if="roomId != null">
        You are already in a room. <RouterLink :to="'/rooms/' + roomId">Go to room</RouterLink>
      </div>
      <v-card v-else>
        <v-card-title>Create a room</v-card-title>
        <v-card-text>
          <v-alert v-if="error" type="error">{{error}}</v-alert>
          <v-text-field label="Room name" v-model="roomName" :rules="roomNameRules" />
          <v-text-field type="password" label="Password (optional)" v-model="password" :rules="passwordRules"
          placeholder="Password is optional; leave blank for public room"
          />
          <v-btn color="success" @click="create">Create</v-btn>
        </v-card-text>
      </v-card>
    </div>
  </v-container>
</template>

<script>
  import { useAuthStore } from '@/store/auth'
  import { RouterLink } from 'vue-router'
  import { post } from '@/requests'
  import { ref, onMounted } from 'vue'
  export default {
    name: "CreateRoom",
    data() {
      return {
        page: 0,
        auth: useAuthStore(),
        roomId: null,
        roomName: "",
        password: "",
        error: null,
        passwordRules: [
          v => (v && v.length >= 4 || v.length === 0) || 'Password must be at least 4 characters',
        ],
        roomNameRules: [
          v => !!v || 'Room name is required',
          v => (v && v.length >= 4) || 'Room name must be at least 4 characters',
        ],
      }
    },
    methods: {
      create() {
        console.log('Create room:\n\tName:' + this.roomName + '\n\tPassword:' + this.password);
        post('rooms', {
          name: this.roomName,
          password: this.password,
        }).then((response) => {
          console.log(response);
          this.$router.push('/rooms/' + response.data.id);
        }).catch((error) => {
          console.error(error);
          let errorMessage = error?.response?.data ?? "No response from server";
          console.error('Failed to create room: ' + errorMessage);
          this.error = errorMessage;
        });
        
      },
    },
    async mounted() {
      if (this.auth.isInAnyRoom) {
        this.roomId = this.auth.getRoomId;
        console.log("User is already in room:" + this.roomId);
      }
      console.log(`the CreateRoom component is now mounted. (room: ${this.roomId}))`)
    },
}
</script>
