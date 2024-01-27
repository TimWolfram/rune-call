<template>
	<v-card class="d-flex flex-column align-center justify-center" rounded="lg" max-width="120">
			<p class="text-body-1 truncate-text text-wrap pa-3">
				{{ name }}
			</p>
		<Card class="ma-1" :card="card"/>
		<br/>
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
