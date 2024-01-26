<template>
  <v-container v-if="roomId != null">
    <p>You are already in a room. <router-link :to="'rooms/' + roomId">Go to room</router-link></p>
  </v-container>
  <v-container v-else-if="!auth.loggedIn">
    <p>To browse rooms, you must be <router-link to="/login">logged in.</router-link></p>
  </v-container>
  <v-container v-else-if="error === null">
    <v-alert type="info">Loading...</v-alert>
  </v-container>
  <v-container v-else-if="error === true">
    <v-alert type="error">Could not reach server; try again later!</v-alert>
  </v-container>
  <v-container v-else>
    <RoomList :rooms=rooms />
    <v-pagination v-model="page" :length="pageCount" @update:model-value="update" />
  </v-container>
</template>

<!-- eslint-disable vue/multi-word-component-names -->
<script>
import RoomList from '@/components/troefcall/room/RoomList.vue'
import { get } from '@/requests'
import { useAuthStore } from '@/store/auth'

export default {
  name: "Rooms",
  components: {
    RoomList
  },
  data() {
    return {
      rooms: [],
      page: 1,
      pageCount: 0,
      auth: useAuthStore(),
      roomId: null,
      error: null
    }
  },

  async mounted() {
    console.log(`the rooms component is now mounted.`)
    this.update();
  },
  methods: {
    async update() {
      let inRoom = await this.auth.isInAnyRoom;
      if (inRoom == true) {
        this.roomId = this.auth.getRoomId;
        console.log("User is already in room:" + this.roomId);
      }
      if (this.page < 1) {
        this.page = 1;
        console.warn('Cannot go to previous page; already at first page');
        return;
      }
      let pageUrl = `rooms/page/${this.page - 1}?public=true`;
      this.error = null;
      await get(pageUrl)
        .then(response => {
          let data = response.data;
          //response is tuple of (pageCount, rooms)
          this.pageCount = data[0];
          this.rooms = data[1];
          //print rooms info to console as json
          console.log("pageCount: " + this.pageCount + "\nrooms: \n" + JSON.stringify(this.rooms));
          this.error = false;
        })
        .catch(error => {
          console.error('Failed to fetch rooms:', error);
          this.error = true; // API call failed
        });
    },

    async nextPage() {
      if (this.page >= this.pageCount) {
        this.page = this.pageCount;
        console.warn('Cannot go to next page; already at last page');
        return;
      }
      this.page++;
      await this.update();
    },

    async previousPage() {
      if (this.page <= 0) {
        this.page = 0;
        console.warn('Cannot go to previous page; already at first page');
        return;
      }
      this.page--;
      await this.update();
    }
  },
}
</script>
