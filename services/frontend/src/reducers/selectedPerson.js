const defaultState = null

export default function selectedPerson(state = defaultState, action) {
	switch (action.type) {
		case 'SELECT_PERSON':
			return action.payload.id

		case 'DESELECTED_PERSON':
			return null

		default:
			return state
	}
}
