<template>
  <v-app-bar flat>
    
    <!-- title -->
    <v-app-bar-title>
      <v-icon icon="mdi-cards" /> Tjall
    </v-app-bar-title>
    
    <v-spacer></v-spacer>

    <!-- settings button -->
    <v-btn icon @click="settingsDrawer = !settingsDrawer">
      <v-icon icon="mdi-cog" />
    </v-btn>

  </v-app-bar>
  
  <!-- Settings drawer -->
  <v-navigation-drawer v-model="settingsDrawer" absolute temporary right>
    <v-switch v-model="lightMode" label="Dark/Light" @update:model-value="toggleTheme"/>
    <v-text-field v-model="displayName" label="Display Name"/>
  </v-navigation-drawer>
</template>

<script setup>

  import { ref, onMounted } from 'vue';
  import Cookies from 'js-cookie';
  import { useTheme } from 'vuetify'

  let settingsDrawer = ref(false);
  let lightMode = ref(false);
  let displayName = ref('');

  onMounted(() => {

    displayName.value = Cookies.get('displayName') || '';
  });


  const theme = useTheme()

  function toggleTheme () {
    let currentThemeName = theme.global.name.value;
    console.log('Current theme: ' + currentThemeName);
    Cookies.set('lightMode', currentlyLightMode);
    setLightMode(currentlyLightMode);
  }
  function setCurrentLightMode(light) {
    const LIGHT_MODE_COOKIE = Cookies.get('lightMode');
    const LIGHT_MODE_ON = LIGHT_MODE_COOKIE === 'light';
    lightMode.value = LIGHT_MODE_ON;
    console.log('Light mode cookie: ' + LIGHT_MODE_COOKIE +  (LIGHT_MODE_ON ? ' (light)' : ' (dark)'));
    setLightMode(LIGHT_MODE_ON);
    const newLocal = light ? 'light' : 'dark';
    console.log('Setting theme to: ' + newLocal);
    theme.global.name.value = newLocal;
  }

</script>
