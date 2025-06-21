// src/icp_crud_dapp_assets/src/App.jsx
import React, { useState, useEffect } from "react";
import { createActor } from "../declarations/icp_crud_dapp";

const actor = createActor();

function App() {
  const [items, setItems] = useState([]);
  const [content, setContent] = useState("");
  const [updateId, setUpdateId] = useState(null);

  useEffect(() => {
    loadItems();
  }, []);

  const loadItems = async () => {
    const data = await actor.getItems();
    setItems(data);
  };

  const handleSubmit = async () => {
    if (updateId) {
      await actor.updateItem(updateId, content);
      setUpdateId(null);
    } else {
      await actor.createItem(content);
    }
    setContent("");
    loadItems();
  };

  const handleDelete = async (id) => {
    await actor.deleteItem(id);
    loadItems();
  };

  const handleEdit = (item) => {
    setContent(item.content);
    setUpdateId(item.id);
  };

  return (
    <div style={{ padding: "2rem" }}>
      <h2>ICP CRUD DApp</h2>
      <input
        value={content}
        onChange={(e) => setContent(e.target.value)}
        placeholder="Enter item..."
      />
      <button onClick={handleSubmit}>{updateId ? "Update" : "Add"}</button>

      <ul>
        {items.map((item) => (
          <li key={item.id}>
            #{item.id}: {item.content}
            <button onClick={() => handleEdit(item)}>✏️</button>
            <button onClick={() => handleDelete(item.id)}>🗑️</button>
          </li>
        ))}
      </ul>
    </div>
  );
}

export default App;
