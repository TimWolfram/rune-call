<!--eslint-disable vue/multi-word-component-names -->
<template>
    <!-- login form -->
    <v-container>
        <v-row>
            <v-col cols="12" sm="6" md="4">
                <v-card>
                    <v-card-title>
                        <span class="headline">Login</span>
                    </v-card-title>
                    <!-- login/register errors -->
                    <v-alert v-if="loginError" type="error">
                        Failed to login: {{ loginError }}
                    </v-alert>
                    <v-card-text>
                        <v-form @keyup.enter="login">
                            <v-text-field v-model="username" prepend-icon="mdi-account" name="username" label="Username" type="text"/>
                            <v-text-field v-model="password" prepend-icon="mdi-lock" name="password" label="Password" type="password"/>
                        </v-form>
                    </v-card-text>
                    <v-card-actions>
                        <v-spacer></v-spacer>
                        <v-btn color="primary" @click="login">Login</v-btn>
                    </v-card-actions>
                    <v-card-text>
                        Don't have an account yet? 
                        <router-link to="register">Register new account</router-link>
                    </v-card-text>
                    <v-alert v-if="auth.loggedIn" type="success">
                        <p>
                            You are logged in as {{ auth.getDisplayName }}. Redirecting to home ... (or <router-link to="/">click here</router-link>)
                        </p>    
                    </v-alert>
                </v-card>
            </v-col>
        </v-row>
    </v-container>
</template>

<script setup>

import { ref, onMounted } from 'vue';
import router from '@/router';
import { useAuthStore } from '@/store/auth';

const username = ref("");
const password = ref("");
const loginError = ref(null);
const auth = useAuthStore();

function login() {
    console.log('Login:\n\tUsername:' + username.value + '\n\tPassword:' + password.value);
    auth.login(username.value, password.value)
    .then((response) => {
        loginError.value = null;
        //redirect to home
        router.push('/');
    }).catch((error) => {
        console.error(error);
        let errorMessage = error?.response?.data ?? "No response from server";
        loginError.value = errorMessage;
        console.error('Failed to login: ' + errorMessage);
    });
}

//on mounted
onMounted(() => {
    console.log('Mounted login form');
});

</script>