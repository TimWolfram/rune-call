<!-- eslint-disable vue/multi-word-component-names -->
<template>
  <v-container v-if="error === null">
    <v-alert type="info">Loading...</v-alert>
  </v-container>
  <v-container v-else-if="error === false">
    <RoomList :rooms=rooms />
  </v-container>
  <v-container v-else>
    <v-alert type="error">Could not reach server; try again later!</v-alert>
  </v-container>
</template>

<script>
  import { ref, onMounted } from 'vue';
  import RoomList from '@/components/troefcall/RoomList.vue'
  import {get} from '@/requests'
  import axios from 'axios'

  export default {
    name: "Rooms",
    components: {
      RoomList
    },
    data() {
      return {
        rooms: [],
        page: 0,
        error: null
      }
    },
    methods: {
      async update() {
        try {
          let pageUrl = 'rooms/page/' + this.page;
          let response = await get(pageUrl);
          if (!response.status === 200) {
            throw new Error('Server response was not ok');
          }
          let newRooms = response.data;
          //print rooms to console as json
          console.log("rooms: \n" + JSON.stringify(newRooms));
          this.rooms = newRooms;
          this.error = false; // API call was successful
        } catch (error) {
          console.error('Failed to fetch rooms:', error);
          this.error = true; // API call failed
        }
      },
      async nextPage() {
        this.page++;
        await this.update();
      },
      async previousPage() {
        if (this.page > 0) {
          this.page--;
          await this.update();
        }
      }
    },
    mounted() {
      console.log(`the component is now mounted.`)
      this.update();
    }
  }
</script>
