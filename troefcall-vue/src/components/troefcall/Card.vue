<template>
    <v-card>
        <p v-if="pref.preferSimpleCard === ''">
            <!-- show card suit and value with unicode -->
            {{ card.suit }} {{ card.value }}
        </p>
    </v-card>
</template>

<script>
