<!-- eslint-disable vue/multi-word-component-names -->
<template>
  <CardList :rooms=rooms />
</template>

<script setup>
  import CardList from '@/components/troefcall/CardList.vue'
  import { ref, onMounted } from 'vue';
  import { HOST } from '@/constants';
  let rooms = ref([]); // reactive
  onMounted(async () => {
    const response = await fetch(HOST);
    rooms.value = await response.json(); // Vue knows that rooms has changed
  });
</script>
