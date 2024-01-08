<template>
  <div class="d-flex flex-wrap justify-space-between">
    <CardListItem v-for="card in props.cards" :key="card.id" :card="card" @click="active = true"/>
  </div>
</template>

<script setup>
  import CardListItem from '@/components/troefcall/CardListItem.vue';
  import { mounted } from 'vue';
  import { defineProps } from 'vue'
  const props = defineProps({
    cards: Array
  });
  mounted()
  {
    setInterval(() => {
      //todo: update cards
    }, 1000);
  }
</script>
