<template>
  <v-app-bar flat>
    <v-app-bar-title>
      <v-icon icon="mdi-cards" /> Tjall
    </v-app-bar-title>
    <v-spacer></v-spacer>
    <v-btn icon @click="drawer = !drawer">
      <v-icon icon="mdi-cog" />
    </v-btn>
  </v-app-bar>
  <v-navigation-drawer v-model="drawer" absolute temporary right>
    <v-switch v-model="darkMode" label="Dark Mode" @update:model-value="toggleTheme"/>
    <v-text-field v-model="displayName" label="Display Name"/>
  </v-navigation-drawer>
</template>

<script setup>

  import { ref, onMounted } from 'vue';
  import Cookies from 'js-cookie';

  let drawer = ref(false);
  let darkMode = ref(false);
  let displayName = ref('');

  onMounted(() => {
    setLightMode(Cookies.get('darkMode'));
    displayName.value = Cookies.get('displayName') || '';
  });

  import { useTheme } from 'vuetify'

  const theme = useTheme()

  function toggleTheme () {
    let currentThemeName = theme.global.name.value;
    let currentTheme = currentThemeName === 'light' ? false : true;
    Cookies.set('darkMode', currentTheme);
    setLightMode(!currentTheme);
  }
  function setLightMode(light){
    theme.global.name.value = light ? 'light' : 'dark'
  }

</script>
