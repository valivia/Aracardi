import { Role } from "@prisma/client";
import React from "react";
import styles from "./user.module.scss";
import { BsGear } from "react-icons/bs";
import { Avatar } from "@components/global/avatar";

const User: React.FC<Props> = ({ user, role, canEdit }) => {
  return (
    <li className={styles.main}>
      <figure className={styles.avatar}><Avatar id={user.avatar_id} /></figure>
      <p className={styles.name}>{user.name}</p>
      {/* TODO roles */}
      <p className={styles.role}>{role}</p>
      {canEdit && <div className={styles.icon}><BsGear /></div>}
    </li>
  );
};

export { User };

interface Props {
  user: { name: string, avatar_id?: string | null }
  role: Role;
  canEdit?: boolean;
}
