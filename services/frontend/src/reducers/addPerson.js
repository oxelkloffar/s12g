const defaultState = null
const emptyValues = {
	"id": "",
	"gender": "",
	"parents": [],
	"children": [],
	"siblings": [],
	"spouses": []
}

export default function addPerson(state = defaultState, action) {
	switch (action.type) {
		case 'START_ADDING_RELATIVE':
			return {
				...emptyValues,
				relative: action.payload.relatedTo
			}

		case 'ABORT_ADDING_PERSON':
			return defaultState

		default:
			return state
	}
}
