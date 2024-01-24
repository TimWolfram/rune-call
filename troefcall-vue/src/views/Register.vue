<!-- eslint-disable vue/multi-word-component-names -->
<template>
    <!-- login form -->
    <v-container>
        <v-row>
            <v-col cols="12" sm="6" md="4">
                <v-card>
                    <v-card-title>
                        <span class="headline">Register new account</span>
                    </v-card-title>
                    <!-- login/register errors -->
                    <v-alert v-if="error" type="error">
                        Failed to register: {{ error }}
                    </v-alert>
                    <v-card-text>
                        <v-form @keyup.enter="register">
                            <v-text-field 
                                v-model="username"
                                :rules="usernameRules"
                                prepend-icon="mdi-account" 
                                name="username" 
                                label="Username" 
                                type="text"/>
                            <v-text-field 
                                v-model="password" 
                                :rules="passwordRules"
                                prepend-icon="mdi-lock" 
                                name="password" 
                                label="Password" 
                                type="password"/>
                        </v-form>
                    </v-card-text>
                    <v-card-actions>
                        <v-spacer></v-spacer>
                        <v-btn color="primary" @click="register">Register new account</v-btn>
                    </v-card-actions>
                    <v-card-text>
                        Already have an account? <router-link to="login">Log in</router-link>
                    </v-card-text> 
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
const error = ref(null);
const auth = useAuthStore();
const usernameRules =  [
    value => {
        if (value?.length >= 3) return true
        return 'Username must be at least 3 characters.'
    },
];
const passwordRules =  [
    value => {
        if (value?.length >= 8) return true
        return 'Password must be at least 8 characters.'
    },
];

function register() {
    console.log('Register:\n\tUsername:' + username.value + '\n\tPassword:' + password.value);
    auth.register(username.value, password.value)
    .then(response => {
        console.log('Register response: ' + JSON.stringify(response.data));
        //go back
        router.push('/');
    }).catch(error => {
        console.error('Failed to register: ' + JSON.stringify(error.response.data));
    });
}

//on mounted
onMounted(() => {
    console.log('Mounted register form');
});

</script>