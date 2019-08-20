import React, { useState, useCallback } from 'react';
import { useSelector, useDispatch } from 'react-redux'
import PinchZoomPan from 'pinch-zoom-pan';
import ReactFamilyTree from 'react-family-tree';
import FamilyNode from '../FamilyNode/FamilyNode';
import styles from './AncestryTree.module.css';

const myID = 'kuVISwh7w';

const WIDTH = 70;
const HEIGHT = 80;

export default React.memo(
  function AncestryTree() {
    const [rootId, setRootId] = useState(myID);
    const onResetClick = useCallback(() => setRootId(myID), []);
    const nodes = useSelector(state => state)

    return (
      <div className={styles.root}>
        <header className={styles.header}>
          <h1 className={styles.title}>
            FamilyTree demo
          </h1>
          <a href="https://github.com/SanichKotikov/react-family-tree-example">GitHub</a>
        </header>
        <PinchZoomPan
          debug
          captureWheel
          min={0.5}
          max={2.5}
          className={styles.wrapper}
        >
          <ReactFamilyTree
            nodes={nodes}
            rootId={rootId}
            width={WIDTH}
            height={HEIGHT}
            canvasClassName={styles.tree}
            renderNode={(node) => (
              <FamilyNode
                key={node.id}
                node={node}
                isRoot={node.id === rootId}
                onSubClick={setRootId}
                style={{
                  width: WIDTH,
                  height: HEIGHT,
                  transform: `translate(${node.left * (WIDTH / 2)}px, ${node.top * (HEIGHT / 2)}px)`,
                }}
              />
            )}
          />
        </PinchZoomPan>
        {rootId !== myID && (
          <div className={styles.reset} onClick={onResetClick}>
            Reset
          </div>
        )}
      </div>
    );
  }
);
