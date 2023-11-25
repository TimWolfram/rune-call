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
    lightMode.value = getLightmodeFromCookies();
    setTheme(lightMode.value);
    displayName.value = Cookies.get('displayName') || '';
  });


  const theme = useTheme()

  function getLightmodeFromCookies() {
    const LIGHT_MODE_COOKIE = Cookies.get('lightMode');
    const LIGHT_MODE_ON = LIGHT_MODE_COOKIE === 'true';
    console.log('Current light mode cookie: ' + LIGHT_MODE_COOKIE +  (LIGHT_MODE_ON ? ' (light)' : ' (dark)'));
    return LIGHT_MODE_ON;
  }
  
  function toggleTheme() {
    console.log('Toggled theme: ' + lightMode.value);
    setTheme(lightMode.value);
  }

  function setTheme (lightMode) {
    console.log('Setting theme to ' + (lightMode ? 'light' : 'dark'));
    let currentThemeName = lightMode ? 'light' : 'dark';
    console.log('Current theme: ' + currentThemeName);
    Cookies.set('lightMode', lightMode);
    theme.global.name.value = currentThemeName;
  }


</script>