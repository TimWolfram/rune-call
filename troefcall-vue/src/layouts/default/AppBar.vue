<template>
  <v-app-bar flat>

    <!-- title -->
    <v-btn variant="plain" to="/">
      <v-app-bar-title>
        <v-icon icon="mdi-cards" /> Tjall
      </v-app-bar-title>
    </v-btn>

    <v-spacer/>

    <!-- settings button -->
    <v-btn icon @click="settingsDrawer = !settingsDrawer">
      <v-icon icon="mdi-cog" />
    </v-btn>

  </v-app-bar>

  <!-- Settings drawer -->
  <v-navigation-drawer v-model="settingsDrawer" location="right" temporary>
    <v-container>
      <v-card class="ma-1 pa-0">
        <v-switch v-model="lightMode" label="Dark/Light" @update:model-value="toggleTheme" />
      </v-card>
      <v-card class="ma-1 pa-1">
        <div v-if="!auth.loggedIn">
          <v-btn size="large" text="Log in" block to="/login" />
        </div>
        <div v-else>
          <v-card-text class="text-subtitle-2" label="Username">
            Hello, {{ auth.getUsername }}!
          </v-card-text>
          <v-text-field v-model="displayName" label="Display Name" :rules="displayNameRules" />
          <v-btn size="large" class="pa-3" block color="success" text="Save Display Name" @click="saveDisplayName" />
          <br/>
          <v-btn size="large" class="pa-3" block color="error" text="Log out" @click="auth.logout" />
        </div>
        <v-container v-if="prefStore.isTesting">
          <p>These are buttons for testing room 0, remove when not testing</p>
          <v-btn size="large" class="ma-1" block color="success" text="Log in as admin" @click="auth.login('admin', 'adminpw!')" />
          <v-btn size="large" class="ma-1" block color="success" text="Log in as user99" @click="auth.login('user99', 'userpw!')" />
          <v-btn size="large" class="ma-1" block color="success" text="Log in as user98" @click="auth.login('user98', 'userpw!')" />
          <v-btn size="large" class="ma-1" block color="success" text="Log in as user97" @click="auth.login('user97', 'userpw!')" />
        </v-container>
      </v-card>
    </v-container>
  </v-navigation-drawer>
</template>

<script setup>
import { ref, onMounted, watch } from 'vue';
import { useTheme } from 'vuetify'
import { useAuthStore } from '@/store/auth';
import { usePreferencesStore } from '@/store/preferences';

const settingsDrawer = ref(false);
const lightMode = ref(false);
const displayName = ref('');
const displayNameRules = [
  v => (v && v.length <= 20) || 'Display name must be at most 20 characters',
];

const prefStore = usePreferencesStore();
let auth = useAuthStore();
const theme = useTheme()

onMounted(() => {
  applyCurrentTheme();
  auth = useAuthStore();
  displayName.value = auth.getDisplayName;
  console.log('Mounted app bar for user: ' + displayName.value);
});
watch(() => auth.user, 
            () => { 
              displayName.value = auth.getDisplayName;
              console.log('Updated display name to: ' + displayName.value);
            }
);

function toggleTheme() {
  const newLigntMode = lightMode.value;
  prefStore.setLightmodePreference(newLigntMode);
  applyTheme(newLigntMode);
}

function applyCurrentTheme() {
  const LIGHTMODEPREF = prefStore.getLightmodePreference;
  lightMode.value = LIGHTMODEPREF;
  applyTheme(LIGHTMODEPREF);
}

function applyTheme(LightModePref) {
  const CURRENTTHEMENAME = LightModePref ? 'light' : 'dark';
  console.log('Applying theme: ' + CURRENTTHEMENAME);
  theme.global.name.value = CURRENTTHEMENAME;
}

function saveDisplayName() {
  auth.setDisplayName(displayName.value);
  console.log('Saved display name as: ' + displayName.value);
}

</script>