import React, { useState } from 'react'
import { useSelector, useDispatch } from 'react-redux'
import Dropdown from 'react-dropdown'
import 'react-dropdown/style.css'
import styles from './AddPerson.module.css'

const relativeOptions = [
	{ value: 'parent', label: 'Parent' },
	{ value: 'child', label: 'Child' },
	{ value: 'sibling', label: 'Sibling' },
	{ value: 'partner', label: 'Partner' }
]
const defaultOption = relativeOptions[0]

function AddPerson(){
	const dispatch = useDispatch()
	const addPersonState = useSelector(state => state.addPerson)
	const relatedPerson = useSelector(state => state.persons
		.filter(person => person.id === (addPersonState ? addPersonState.relative : null))
	)
	const [name, setName] = useState(addPersonState ? addPersonState.name : null)
	const [relation, setRelation] = useState(defaultOption.value)

	if (addPersonState === null) {
		return <></>
	}

	const nameOfRelative = relatedPerson[0].name

	const finishAddingPerson = () => {
		dispatch({ type: 'ABORT_ADDING_PERSON' })
		dispatch({ type: 'FINISH_ADDING_RELATIVE', payload: {
			name,
			relatedTo: addPersonState.relative,
			relation
		} })
	}

	return(
		<>
			<div className={styles.bg} onClick={() => dispatch({ type: 'ABORT_ADDING_PERSON' })}></div>
			<div className={styles.modal}>
				<p>Name</p>
				<input type="text" value={name} onChange={(e) => setName(e.target.value)} />

				<p>
					Relation to
					<b> {nameOfRelative}</b>
				</p>
				<Dropdown
					options={relativeOptions}
					value={defaultOption}
					onChange={e => setRelation(e.value)}
				/>

				<br /><br />
				<button onClick={finishAddingPerson}>
					Add this person!
				</button>
			</div>
		</>
	)
}

export default AddPerson
