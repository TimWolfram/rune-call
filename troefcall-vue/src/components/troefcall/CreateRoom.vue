<template>
    <v-container>
        <p>Create Room</p>
    </v-container>
</template>

<script>
  export default {
    name: "CreateRoom",
    
    data() {
      return {
        rooms: [],
        page: 0,
        loginErr: null,
        loginUser: null
      }
    },
    methods: {
    },
    mounted() {
      console.log(`the CreateRoom component is now mounted.`)
    },
}
</script>
