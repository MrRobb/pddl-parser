(define (domain collaborative-cloth-piling)

    (:requirements :strips :typing :durative-actions :numeric-fluents)

    (:types
        robot human - agent
        garment pile agent - physical-object
        garment-type - concept
        concept - social-object
        social-object physical-object - object
        object - entity
        entity
    )

    (:predicates
        (grasped-by ?o - object ?a - agent)
        (graspable ?o - object)
        (free-to-manipulate ?a - agent)
        (on-pile ?g - garment ?p - pile)
        (piled ?g - garment)
        (supported ?g - garment)
        (lifted ?g - garment)
        (folded ?g - garment)
        (unfolded ?g - garment)
    )

    (:functions
        (grasp-time ?a - agent)
        (current-number-of-garments-on-pile ?p - pile)
        (target-number-of-garments-on-pile ?p - pile)
    )

    (:durative-action grasp-folded-garment
        :parameters (?g - garment ?a - agent)
        :duration (= ?duration (grasp-time ?a))
        :condition (and
            (at start (free-to-manipulate ?a))
            (at start (folded ?g))
            (at start (graspable ?g))
        )
        :effect (and
            (at start (not (free-to-manipulate ?a)))
            (at start (not (graspable ?g)))
            (at end (grasped-by ?g ?a))
        )
    )

    (:durative-action grasp-unfolded-garment
        :parameters (?g - garment ?h - human)
        :duration (= ?duration 100)
        :condition (and
            (at start (free-to-manipulate ?h))
            (at start (unfolded ?g))
            (at start (graspable ?g))
        )
        :effect (and
            (at start (not (free-to-manipulate ?h)))
            (at start (not (graspable ?g)))
            (at end (grasped-by ?g ?h))
        )
    )

    (:durative-action lift ; aka removetablecontact
        :parameters (?g - garment ?a - agent)
        :duration (= ?duration 100)
        :condition (and
            (at start (grasped-by ?g ?a))
            (at start (supported ?g))
        )
        :effect (and
            (at end (not (supported ?g)))
            (at end (lifted ?g))
        )
    )

    (:durative-action pile-garment ; aka transfer
        :parameters (?g - garment ?p - pile ?t - garment-type ?a - agent)
        :duration (= ?duration (grasp-time ?a))
        :condition (and
            (at start (grasped-by ?g ?a))
            (at start (lifted ?g))
            (at start (folded ?g))
        )
        :effect (and
            (at start (not (grasped-by ?g ?a)))
            (at end (graspable ?g))
            (at end (free-to-manipulate ?a))
            (at end (piled ?g))
            (at end (on-pile ?g ?p))
            (at end (increase
                    (current-number-of-garments-on-pile ?p)
                    1))
        )
    )

    (:durative-action fold-garment
        :parameters (?g - garment ?h - human)
        :duration (= ?duration 100)
        :condition (and
            (at start (unfolded ?g))
            (at start (lifted ?g))
            (at start (grasped-by ?g ?h))
        )
        :effect (and
            (at end (free-to-manipulate ?h))
            (at end (not (unfolded ?g)))
            (at end (not (lifted ?g)))
            (at end (not (grasped-by ?g ?h)))
            (at end (graspable ?g))
            (at end (folded ?g))
            (at end (supported ?g))
        )
    )

    (:durative-action grasp-pile-of-garments
        :parameters (?p - pile ?h - human)
        :duration (= ?duration 100)
        :condition (and
            (at start (free-to-manipulate ?h))
            (at start (= (current-number-of-garments-on-pile ?p) (target-number-of-garments-on-pile ?p)))
            (at start (graspable ?p))
        )
        :effect (and
            (at start (not (free-to-manipulate ?h)))
            (at start (not (graspable ?p)))
            (at end (grasped-by ?p ?h))
        )
    )
)