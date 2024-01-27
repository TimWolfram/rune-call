<template>
	<v-card class="d-flex align-content-center">
		<v-card-text class="text-body-1">{{ name }}</v-card-text>
		<Card :card="card"/>
	</v-card>
</template>

<script>
import Card from '@/components/troefcall/game/Card.vue'

export default {
	name: "PlayerCard",
	props: {
		name: {
			type: String,
			required: true,
		},
		card: {
			type: Object,
			required: false,
		},
	},
	components: {
		Card,
	},
}
</script>
