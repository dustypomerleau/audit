module default {
    scalar type EmailType extending str {
        # HTML5 allows dotless domains, but ICANN doesn't, so prohibit here
        constraint regexp("^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)+$");
    }

    # enums
    scalar type Side extending enum<Right, Left>;
    scalar type Formula extending enum<Barrett, Kane>;
    scalar type Adverse extending enum<Rhexis, Pc, Zonule, Other>;
    #...

    # object types
    type Surgeon {}
    type Case {}
    #...
}
