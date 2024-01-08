<template>
  <v-container>
    <div v-if="!auth.loggedIn">
      You must <RouterLink to="/login">log in</RouterLink> before creating a room.
    </div>
    <div v-else>
      <p>Create Room (TODO)</p>
    </div>
  </v-container>
</template>

<script>
  import { useAuthStore } from '@/store/auth'
  export default {
    name: "CreateRoom",
    
    data() {
      return {
        page: 0,
        auth: useAuthStore(),
      }
    },
    methods: {
      create() {
        console.error('TODO: create room');
      },
    },
    mounted() {
      console.log(`the CreateRoom component is now mounted.`)
    },
}
</script>
