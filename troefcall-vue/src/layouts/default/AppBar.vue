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
  <v-navigation-drawer v-model="settingsDrawer" location="right" temporary>
    <v-switch v-model="lightMode" label="Dark/Light" @update:model-value="toggleTheme"/>
    <v-text-field v-model="displayName" label="Display Name" @update:model-value="changeDisplayName"/>
  </v-navigation-drawer>
</template>

<script setup>

  import { ref, onMounted } from 'vue';
  import Cookies from 'js-cookie';
  import { useTheme } from 'vuetify'

  const displayNameCookie = 'displayName';
  const lightModeCookie = 'lightMode';

  let settingsDrawer = ref(false);
  let lightMode = ref(false);
  let displayName = ref('');

  onMounted(() => {
    getLightmodeFromCookies();
    applyTheme(lightMode.value);
    displayName.value = Cookies.get(displayNameCookie) || '';
  });


  const theme = useTheme()

  function getLightmodeFromCookies() {
    const LIGHT_MODE_ON = Cookies.get(lightModeCookie) === 'true';
    console.log('Current light mode cookie: ' +  (LIGHT_MODE_ON ? ' (light)' : ' (dark)'));
    lightMode.value = LIGHT_MODE_ON;
  }
  
  function toggleTheme() {
    console.log('Toggled theme: ' + lightMode.value);
    applyTheme();
  }

  function applyTheme () {
    console.log('Setting theme to ' + (lightMode.value ? 'light' : 'dark'));
    Cookies.set(lightModeCookie, lightMode.value);
    let currentThemeName = lightMode.value ? 'light' : 'dark';
    console.log('Current theme: ' + currentThemeName);
    theme.global.name.value = currentThemeName;
  }

  function changeDisplayName() {
    console.log('Changed display name to ' + displayName.value);
    Cookies.set(displayNameCookie, displayName.value);
  }


</script>