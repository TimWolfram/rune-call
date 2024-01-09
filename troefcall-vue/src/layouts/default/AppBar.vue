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
    <v-switch v-model="lightMode" label="Dark/Light" @update:model-value="toggleTheme" class="ml-3"/>
    
    <div v-if="!auth.loggedIn">
      <v-btn label="Log in" to="login"/>
    </div>
    <div v-else>
    <v-btn label="Log out" @click="auth.logOut"/>
      <v-text-field v-model="displayName" label="Display Name"/>
      <v-btn label="Save Display Name" @click="saveDisplayName"/>
    </div>
  </v-navigation-drawer>
</template>

<script setup>

  import { ref, onMounted } from 'vue';
  import { useTheme } from 'vuetify'
  import { useAuthStore } from '@/store/auth';
  import { usePreferencesStore } from '@/store/preferences';


  let settingsDrawer = ref(false);
  let lightMode = ref(false);

  let displayName = ref('');
  let prefStore = usePreferencesStore();
  let auth = useAuthStore();
  
  onMounted(() => {
    applyCurrentTheme();
    displayName.value = auth.getDisplayName;
  });


  const theme = useTheme()

  function toggleTheme() {
    const NEWLIGHTMODEPREF = !prefStore.getLightmodePreference;
    prefStore.setLightmodePreference(NEWLIGHTMODEPREF);
    console.log('Setting theme to ' + (NEWLIGHTMODEPREF ? 'light' : 'dark'));
    applyTheme(NEWLIGHTMODEPREF);
    lightMode.value = NEWLIGHTMODEPREF;
  }

  function applyCurrentTheme () {
    const LIGHTMODEPREF = prefStore.getLightmodePreference;
    applyTheme(LIGHTMODEPREF);
    lightMode.value = LIGHTMODEPREF;
  }

  function applyTheme (LightModePref) {
    const CURRENTTHEMENAME = LightModePref ? 'light' : 'dark';
    console.log('Current theme: ' + CURRENTTHEMENAME);
    theme.global.name.value = CURRENTTHEMENAME;
  }

  function saveDisplayName() {
    auth.setDisplayName(displayName.value);
    console.log('Saved display name as: ' + displayName.value);
  }

</script>